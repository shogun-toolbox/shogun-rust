#include "shogun.hpp"
#include <shogun/util/factory.h>

#include <type_traits>

using namespace shogun;

// taken from cpp reference
template<class... Ts> struct overloaded : Ts... { using Ts::operator()...; };
template<class... Ts> overloaded(Ts...) -> overloaded<Ts...>;

struct version {
	std::unique_ptr<Version> obj;
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
	std::variant<std::shared_ptr<Machine>, std::shared_ptr<Kernel>, std::shared_ptr<Distance>> ptr;

	template <typename T, std::enable_if_t<is_sg_base<T>::value>* = nullptr>
	sgobject(const std::shared_ptr<T>& ptr_): ptr(ptr_) {
		// singleton pattern ensures we only register visitors once
		VisitorRegister::instance();
	}

	~sgobject() = default;

	Any get_parameter(const char* name) const {
		const auto params = std::visit([&name](auto&& arg){return arg->get_params();}, ptr);
		const auto param = params.find(std::string(name));
		if (param != params.end())
			return param->second->get_value();
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
			if (get_type<ReturnType>::type.first != visitor->type)
				error("Type mismatch");
			*val = *static_cast<const RegisterType*>(visitor->m_value);
		}
	);
}

VisitorRegister* VisitorRegister::instance() {
	static auto object = VisitorRegister{};
	return &object; 
}

version_t* create_version() {
	auto* ptr = (version_t*)malloc(sizeof(version_t));
	ptr->obj = std::make_unique<Version>();
	return ptr;
}

void destroy_version(version_t* ptr) {
	if (ptr) {
		ptr->obj.reset();
		free(ptr);
	}
}

const char* get_version_main(version_t* ptr) {
	if (ptr) {
		return ptr->obj->get_version_main();
	}
}

template <typename SGType>
sgobject_result create_helper(const char* name) {
	try {
		auto obj = create<SGType>(name);
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

sgobject_put_result sgobject_put(sgobject_t* ptr, const char* name, const void* value, TYPE type) {
	const auto& param = ptr->get_parameter(name);
	auto visitor = Put_Visitor{value, type};
	try {
		param.visit_with(&visitor);
		return {RETURN_CODE::SUCCESS, nullptr};
	}
	catch(const std::exception& e) {
		return {RETURN_CODE::ERROR, e.what()};
	}
}

SG_TYPE sgobject_derived_type(const sgobject_t* ptr) {
	return std::visit( overloaded {
		[](const std::shared_ptr<Kernel>&){return SG_TYPE::SG_KERNEL;},
		[](const std::shared_ptr<Machine>&){return SG_TYPE::SG_MACHINE;},
		[](const std::shared_ptr<Distance>&){return SG_TYPE::SG_DISTANCE;},
	}, ptr->ptr);
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