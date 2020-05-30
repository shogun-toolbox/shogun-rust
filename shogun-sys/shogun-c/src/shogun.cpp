#include "shogun.hpp"
#include <shogun/util/factory.h>

using namespace shogun;

// taken from cpp reference
template<class... Ts> struct overloaded : Ts... { using Ts::operator()...; };
template<class... Ts> overloaded(Ts...) -> overloaded<Ts...>;

struct version {
	Version* obj;
};

struct C_Visitor {
	std::pair<TYPE, std::string_view> m_type;
	void* m_value;
};

struct Put_Visitor {
	const void* m_value;
	const TYPE type;
};

template <typename>
struct get_type {};

#define STRINGIFY(a) #a

#define DEFINE_TYPE(TYPE_T, TYPE_ENUM)\
template <>\
struct get_type<TYPE_T> {\
	static constexpr std::pair<TYPE, std::string_view> type = {TYPE_ENUM, STRINGIFY(TYPE_T)};\
};

DEFINE_TYPE(int32_t, INT32)
DEFINE_TYPE(int64_t, INT64)
DEFINE_TYPE(float32_t, FLOAT32)
DEFINE_TYPE(float64_t, FLOAT64)
DEFINE_TYPE(std::shared_ptr<SGObject>, SGOBJECT)

#undef DEFINE_TYPE
#undef STRINGIFY

class VisitorRegister {
	template <typename T>
	void register_visitor();
	
	VisitorRegister();

	public:
		static VisitorRegister* instance();
};

struct sgobject {
	std::variant<std::shared_ptr<Machine>, 
			     std::shared_ptr<Kernel>, 
				 std::shared_ptr<Distance>, 
				 std::shared_ptr<Features>,
				 std::shared_ptr<File>,
				 std::shared_ptr<CombinationRule>,
				 std::shared_ptr<Labels>,
				 std::shared_ptr<Evaluation>> ptr;

	template <typename T, std::enable_if_t<is_sg_base<T>::value>* = nullptr>
	sgobject(const std::shared_ptr<T>& ptr_): ptr(ptr_) {
		// singleton pattern ensures we only register visitors once
		VisitorRegister::instance();
	}

	sgobject(const std::shared_ptr<File>& ptr_): ptr(ptr_) {
		VisitorRegister::instance();
	}

	~sgobject() = default;

	std::string get_name() const {
		return std::visit([](auto&& obj) {
			return obj->get_name();
		}, ptr);
	}

	SG_TYPE derived_type() const {
		return std::visit( overloaded {
			[](const std::shared_ptr<Kernel>&){return SG_TYPE::SG_KERNEL;},
			[](const std::shared_ptr<Machine>&){return SG_TYPE::SG_MACHINE;},
			[](const std::shared_ptr<Distance>&){return SG_TYPE::SG_DISTANCE;},
			[](const std::shared_ptr<Features>&){return SG_TYPE::SG_FEATURES;},
			[](const std::shared_ptr<File>&){return SG_TYPE::SG_FILE;},
			[](const std::shared_ptr<CombinationRule>&){return SG_TYPE::SG_COMBINATION_RULE;},
			[](const std::shared_ptr<Labels>&){return SG_TYPE::SG_LABELS;},
			[](const std::shared_ptr<Evaluation>&){return SG_TYPE::SG_EVALUATION;},
		}, ptr);
	}

	Any get_parameter(const char* name) const {
		const auto params = std::visit([&name](auto&& arg){return arg->get_params();}, ptr);
		const auto param = params.find(std::string(name));
		if (param != params.end())
			return param->second->get_value();
		else
			error("Could not find parameter {}::{}", get_name(), name);
	}

	const char* to_string() const {
		auto repr = std::visit([](auto&& arg) {return arg->to_string();}, ptr);
		auto* result = (char*)malloc(sizeof(char*) * repr.size() + 1);
		strcpy(result, repr.c_str());
		return result;
	}
};

VisitorRegister::VisitorRegister () {
	register_visitor<float32_t>();
	register_visitor<float64_t>();
	register_visitor<int32_t>();
	register_visitor<int64_t>();
	register_visitor<Kernel>();
	register_visitor<Machine>();
	register_visitor<Distance>();
	register_visitor<Features>();
	register_visitor<CombinationRule>();
	register_visitor<Labels>();
	register_visitor<Evaluation>();
}

/** Helper function to handle type casting internally.
 * Returns false if it didn't manage to cast lhs to rhs.
 */
template <typename T>
bool internal_type_promotions_compare_types(TYPE rhs, T* val_lhs, const void* val_rhs) {
	if constexpr (std::is_same_v<T, int32_t> || std::is_same_v<T, int64_t>)
	{
		if (rhs == INT32)
			*val_lhs = *static_cast<const int32_t*>(val_rhs);
		else if (rhs == INT64)
			*val_lhs = *static_cast<const int64_t*>(val_rhs);
		else
			return false;
		return true;
	}
	return false;
}

template <typename T>
void VisitorRegister::register_visitor() {
	using RegisterType = std::conditional_t<is_sg_base<T>::value, std::shared_ptr<T>, T>;
	using ReturnType = std::conditional_t<is_sg_base<T>::value, std::shared_ptr<SGObject>, T>;

	Any::register_visitor<RegisterType, C_Visitor>(
		[](RegisterType* val, C_Visitor* visitor) {
			auto* result = new ReturnType;
			*result = *val;
			visitor->m_type = get_type<ReturnType>::type;
			if constexpr (is_sg_base<T>::value)
				visitor->m_value = (void*)new sgobject(*val);
			else
				visitor->m_value = (void*)result;
		}
	);
	Any::register_visitor<RegisterType, Put_Visitor>(
		[](RegisterType* val, Put_Visitor* visitor) {
			if (!internal_type_promotions_compare_types(
				visitor->type, val, visitor->m_value)) {
				// if the types were not casted internally check if types match exactly
				if (get_type<ReturnType>::type.first != visitor->type) {
					error("Type mismatch");
				}
				if constexpr (is_sg_base<T>::value) {
					auto obj = static_cast<const sgobject_t*>(visitor->m_value);
					if (std::holds_alternative<RegisterType>(obj->ptr))
						*val = std::get<RegisterType>(obj->ptr);
					else
						error("SGObject type mismatch");
				}
				else {
					// if we got here types match exactly so can static_cast
					*val = *static_cast<const RegisterType*>(visitor->m_value);
				}
			}
		}
	);

	// automatically registers arithmetic types' corresponding SGMatrix and SGVector
	if constexpr (std::is_arithmetic_v<RegisterType>)
	{
		using MatrixType = SGMatrix<RegisterType>;
		using VectorType = SGVector<RegisterType>;

		Any::register_visitor<VectorType, Put_Visitor>( 
			[](VectorType* val, Put_Visitor* visitor) {
				*val = *static_cast<const VectorType*>(visitor->m_value);
			}
		);

		Any::register_visitor<MatrixType, Put_Visitor>( 
			[](MatrixType* val, Put_Visitor* visitor) {
				*val = *static_cast<const MatrixType*>(visitor->m_value);
			}
		);
	}
}

VisitorRegister* VisitorRegister::instance() {
	static auto object = VisitorRegister{};
	return &object; 
}

version_t* create_version() {
	auto* ptr = (version_t*)malloc(sizeof(version_t));
	ptr->obj = new Version();
	return ptr;
}

void set_parallel_threads(int32_t n_threads) {
	env()->set_num_threads(n_threads);
}

void destroy_version(version_t* ptr) {
	if (ptr) {
		delete ptr->obj;
		free(ptr);
	}
}

const char* get_version_main(version_t* ptr) {
	if (ptr) {
		return ptr->obj->get_version_main();
	}
}

template <typename SGType, typename... Args>
sgobject_result create_helper(Args&&... args) {
	try {
		auto obj = create<SGType>(std::forward<Args>(args)...);
		auto* ptr = new sgobject_t(obj);
		return {RETURN_CODE::SUCCESS, ptr};
	}
	catch (const std::exception& e) {
		sgobject_result result;
		result.return_code = RETURN_CODE::ERROR;
		result.result.error = e.what();
		return result;
	}
}

sgobject_result create_machine(const char* name) {
	return create_helper<Machine>(name);
}

Result train_machine(sgobject_t* machine, sgobject_t* features) {
	if (!std::holds_alternative<std::shared_ptr<Machine>>(machine->ptr))
		return {RETURN_CODE::ERROR, "Expected training to be done with Machine type"};
	if (!std::holds_alternative<std::shared_ptr<Features>>(features->ptr))
		return {RETURN_CODE::ERROR, "Expected training to be done on Features type"};
	try {
		std::get<std::shared_ptr<Machine>>(machine->ptr)->train(
			std::get<std::shared_ptr<Features>>(features->ptr)
		);
		return {RETURN_CODE::SUCCESS, nullptr};
	}
	catch (std::exception& e) {
		return {RETURN_CODE::ERROR, e.what()};
	}
}

template <typename T, typename ResultType=sgobject_result>
std::optional<ResultType> check_type(const sgobject_t* obj, const char* error_msg) {
	if (!std::holds_alternative<std::shared_ptr<T>>(obj->ptr)) {
		ResultType result;
		result.return_code = RETURN_CODE::ERROR;
		result.result.error = error_msg;
		return result;
	}
	return {};
}

sgobject_result apply_machine(sgobject_t* machine, sgobject_t* features) {
	if (auto result = check_type<Machine>(machine, "Expected inference to be done with Machine type"))
		return *result;
	if (auto result = check_type<Features>(features, "Expected inference to be done on Features type"))
		return *result;
	try {
		auto result = std::get<std::shared_ptr<Machine>>(machine->ptr)->apply(std::get<std::shared_ptr<Features>>(features->ptr));
		auto* ptr = new sgobject_t(result);
		return {RETURN_CODE::SUCCESS, ptr};
	}
	catch (const std::exception& e) {
		sgobject_result result;
		result.return_code = RETURN_CODE::ERROR;
		result.result.error = e.what();
		return result;
	} 
}

sgobject_result apply_multiclass_machine(sgobject_t* machine, sgobject_t* features) {
	if (auto result = check_type<Machine>(machine, "Expected inference to be done with Machine type"))
		return *result;
	if (auto result = check_type<Features>(features, "Expected inference to be done on Features type"))
		return *result;
	try {
		auto result = std::get<std::shared_ptr<Machine>>(machine->ptr)->apply_multiclass(std::get<std::shared_ptr<Features>>(features->ptr));
		auto* ptr = new sgobject_t(std::static_pointer_cast<Labels>(result));
		return {RETURN_CODE::SUCCESS, ptr};
	}
	catch (const std::exception& e) {
		sgobject_result result;
		result.return_code = RETURN_CODE::ERROR;
		result.result.error = e.what();
		return result;
	} 
}

sgobject_result create_kernel(const char* name) {
	return create_helper<Kernel>(name);
}

sgobject_result create_distance(const char* name) {
	return create_helper<Distance>(name);
}

void destroy_sgobject(sgobject* ptr) {
	if (ptr) {
		delete ptr;
	}
}

const char* to_string(const sgobject_t* ptr) {
	if (ptr) {
		return ptr->to_string();
	}
}

cvisitor_t* sgobject_get(const sgobject_t* ptr, const char* name) {
	const auto& param = ptr->get_parameter(name);
	auto* visitor = new C_Visitor{};
	param.visit_with(visitor);
	return visitor;
}

Result sgobject_put(sgobject_t* ptr, const char* name, const void* value, TYPE type) {
	const auto& param = ptr->get_parameter(name);
	auto visitor = Put_Visitor{value, type};
	if (type == SGOBJECT)
		visitor.m_value = std::visit([](auto&& obj){return (void*)&obj;}, static_cast<const sgobject_t*>(value)->ptr);
	try {
		param.visit_with(&visitor);
		return {RETURN_CODE::SUCCESS, nullptr};
	}
	catch(const std::exception& e) {
		return {RETURN_CODE::ERROR, e.what()};
	}
}

template <typename T>
SGMatrix<T> create_matrix_with_copy(const T* data, uint32_t rows, uint32_t cols) {
	auto mat = SGMatrix<T>(rows, cols);
	sg_memcpy(mat.matrix, data, rows*cols*sizeof(T));
	return mat;
}

Result sgobject_put_array(sgobject_t* ptr, const char* name, const void* data, uint32_t rows, uint32_t cols, TYPE type) {
	try {
		const auto& param = ptr->get_parameter(name);
		// it's a vector
		if (rows == 0) {
			// create_matrix_with_copy(const T* data, uint32_t rows, uint32_t cols)
			error("SGVector not implemented yet.");
		}
		else {
			switch (type)
			{
			case TYPE::FLOAT32: {
				const auto* casted_data = static_cast<const float32_t*>(data);
				const auto mat = create_matrix_with_copy(casted_data, rows, cols);
				Put_Visitor visitor{(const void*) &mat};
				param.visit_with(&visitor);
			} break;
			case TYPE::FLOAT64: {
				auto* casted_data = static_cast<const float64_t*>(data);
				const auto mat = create_matrix_with_copy(casted_data, rows, cols);
				Put_Visitor visitor{(const void*) &mat};
				param.visit_with(&visitor);
			} break;
			case TYPE::INT32: {
				auto* casted_data = static_cast<const int32_t*>(data);
				const auto mat = create_matrix_with_copy(casted_data, rows, cols);
				Put_Visitor visitor{(const void*) &mat};
				param.visit_with(&visitor);
			} break;
			case TYPE::INT64: {
				auto* casted_data = static_cast<const int64_t*>(data);
				const auto mat = create_matrix_with_copy(casted_data, rows, cols);
				Put_Visitor visitor{(const void*) &mat};
				param.visit_with(&visitor);
			} break;
			default: {
				return {RETURN_CODE::ERROR, "Cannot handle scalar type for SGMatrix"};	
			};
			}
		}
		return {RETURN_CODE::SUCCESS, nullptr};
	}
	catch(const std::exception& e) {
		return {RETURN_CODE::ERROR, e.what()};
	}
}

SG_TYPE sgobject_derived_type(const sgobject_t* ptr) {
	return ptr->derived_type();
}

TYPE get_cvisitor_type(const cvisitor_t* ptr) {
	return ptr->m_type.first;
}

const char* get_cvisitor_typename(const cvisitor_t* ptr) {
	return ptr->m_type.second.data();
}

void* get_cvisitor_pointer(const cvisitor_t* ptr) {
	return ptr->m_value;
}

sgobject_result create_features(const char* name) {
	return create_helper<Features>(name);
}

sgobject_result create_features_from_data(const void* data, uint32_t rows, uint32_t cols, TYPE type) {
	switch (type)
	{
	case TYPE::FLOAT32: {
		const auto* casted_data = static_cast<const float32_t*>(data);
		auto mat = create_matrix_with_copy(casted_data, rows, cols);
		return create_helper<Features>(mat);
	} break;
	case TYPE::FLOAT64: {
		auto* casted_data = static_cast<const float64_t*>(data);
		auto mat = create_matrix_with_copy(casted_data, rows, cols);
		return create_helper<Features>(mat);
	} break;
	case TYPE::INT32: {
		auto* casted_data = static_cast<const int32_t*>(data);
		auto mat = create_matrix_with_copy(casted_data, rows, cols);
		return create_helper<Features>(mat);
	} break;
	case TYPE::INT64: {
		auto* casted_data = static_cast<const int64_t*>(data);
		auto mat = create_matrix_with_copy(casted_data, rows, cols);
		return create_helper<Features>(mat);
	} break;
	default: {
		sgobject_result result;
		result.return_code = RETURN_CODE::ERROR;
		result.result.error = "Cannot create a Features object from provided data";
		return result;	
	};
	}
}

sgobject_result create_features_from_file(const sgobject_t* file) {
	if (auto result = check_type<File>(file, "Expected self to be of type File"))
		return *result;
	return create_helper<Features>(std::get<std::shared_ptr<File>>(file->ptr));
}

sgobject_result create_labels(const char* name) {
	sgobject_result result;
	result.return_code = RETURN_CODE::ERROR;
	result.result.error = "Cannot generate a Labels instance from class name";
	return result;
}

sgobject_result create_labels_from_file(const sgobject_t* file) {
	if (auto result = check_type<File>(file, "Expected self to be of type File"))
		return *result;
	return create_helper<Labels>(std::get<std::shared_ptr<File>>(file->ptr));
}

Result init_kernel(sgobject_t* kernel, sgobject_t* lhs, sgobject_t* rhs) {
	if (!std::holds_alternative<std::shared_ptr<Kernel>>(kernel->ptr)) {
		return Result{RETURN_CODE::ERROR, "Expected self to be Kernel type."};
	}
	if (!std::holds_alternative<std::shared_ptr<Features>>(lhs->ptr)) {
		return Result{RETURN_CODE::ERROR, "Expected lhs to be of type Features"};
	}
	if (!std::holds_alternative<std::shared_ptr<Features>>(rhs->ptr)) {
		return Result{RETURN_CODE::ERROR, "Expected rhs to be of type Features"};
	}
	std::get<std::shared_ptr<Kernel>>(kernel->ptr)->init(
		std::get<std::shared_ptr<Features>>(lhs->ptr),
		std::get<std::shared_ptr<Features>>(rhs->ptr)
	);
	return {RETURN_CODE::SUCCESS, nullptr};
}

sgobject_result create_file(const char* name) {
	return create_helper<File>(name);
}

sgobject_result read_csvfile(const char* filepath) {
	return create_helper<CSVFile>(filepath);
}

sgobject_result create_combination_rule(const char* name) {
	return create_helper<CombinationRule>(name);
}

sgobject_result create_evaluation(const char* name) {
	return create_helper<Evaluation>(name);
}

float64_result evaluate_labels(sgobject* self, sgobject_t* y_pred, sgobject_t* y_true) {
	if (auto result = check_type<Evaluation, float64_result>(self, "Expected self to be of type Evaluation"))
		return *result;
	if (auto result = check_type<Labels, float64_result>(y_pred, "Expected y_pred to be of type Labels"))
		return *result;
	if (auto result = check_type<Labels, float64_result>(y_true, "Expected y_true to be of type Labels"))
		return *result;
	auto result = std::get<std::shared_ptr<Evaluation>>(self->ptr)->evaluate(
		std::get<std::shared_ptr<Labels>>(y_pred->ptr),
		std::get<std::shared_ptr<Labels>>(y_true->ptr)
	);
	return {RETURN_CODE::SUCCESS, result};
}
