#include "shogun.hpp"
#include <shogun/base/Version.h>
#include <shogun/util/factory.h>

#include <type_traits>

using namespace shogun;

struct version {
	std::unique_ptr<Version> obj;
};

struct C_Visitor {
	std::pair<TYPE, std::string_view> m_type;
	void* m_value;
};

template <typename>
struct get_type {};

template <TYPE>
struct get_name {};

#define STRINGIFY(a) #a

#define DEFINE_TYPE(TYPE_T, TYPE_ENUM)\
template <>\
struct get_type<TYPE_T> {\
	static constexpr TYPE value = TYPE_ENUM;\
	static constexpr std::string_view name = STRINGIFY(TYPE_T);\
};\

DEFINE_TYPE(int32_t, INT32)
DEFINE_TYPE(int64_t, INT64)
DEFINE_TYPE(float32_t, FLOAT32)
DEFINE_TYPE(float64_t, FLOAT64)
DEFINE_TYPE(std::shared_ptr<SGObject>, SGOBJECT)

#undef DEFINE_TYPE
#undef STRINGIFY

class VisitorRegister {
	VisitorRegister () {
		register_visitor<float32_t>();
		register_visitor<float64_t>();
		register_visitor<int32_t>();
		register_visitor<int64_t>();
		register_visitor<std::shared_ptr<SGObject>>();
	}

	template <typename T>
	void register_visitor() {
		using ReturnType = std::conditional_t<is_sg_base<T>::value, std::shared_ptr<SGObject>, T>;
		Any::register_visitor<T, C_Visitor>(
			[](auto* val, auto* visitor) {
				auto* result = new ReturnType;
				sg_memcpy(result, val, sizeof(ReturnType));
				visitor->m_type = std::make_pair(get_type<ReturnType>::value, get_type<ReturnType>::name);
				visitor->m_value = (void*)result;
			}
		);
	}

	public:
		static VisitorRegister* instance() {
			static auto object = VisitorRegister{};
			return &object; 
		}
};

struct sgobject {
	std::variant<std::shared_ptr<Machine>, std::shared_ptr<Kernel>, std::shared_ptr<Distance>> ptr;

	template <typename T, std::enable_if_t<is_sg_base<T>::value>* = nullptr>
	sgobject(const std::shared_ptr<T>& ptr_): ptr(ptr_) {
		// singleton pattern ensures we only register visitors once
		VisitorRegister::instance();
	}

	virtual ~sgobject() {
		std::visit([](auto&& arg){arg.reset();}, ptr);
	}

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

sgobject_t* create_machine(const char* name) {
	auto obj = create<Machine>(name);
	auto* ptr = new sgobject_t(obj);
	return ptr;
}

sgobject_t* create_kernel(const char* name) {
	auto obj = create<Kernel>(name);
	auto* ptr = new sgobject_t(obj);
	return ptr;
}

sgobject_t* create_distance(const char* name) {
	auto obj = create<Distance>(name);
	auto* ptr = new sgobject_t(obj);
	return ptr;
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

cvisitor_t* sgobject_get(const sgobject* ptr, const char* name) {
	const auto& param = ptr->get_parameter(name);
	auto* visitor = new C_Visitor{};
	param.visit_with(visitor);
	return visitor;
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