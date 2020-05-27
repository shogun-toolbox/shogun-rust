#include "shogun.hpp"
#include <shogun/base/Version.h>
#include <shogun/util/factory.h>

using namespace shogun;

struct version {
	std::unique_ptr<Version> obj;
};

struct sgobject {
	std::variant<std::shared_ptr<Machine>, std::shared_ptr<Kernel>> ptr;

	template <typename T, std::enable_if_t<is_sg_base<T>::value>* = nullptr>
	sgobject(const std::shared_ptr<T>& ptr_): ptr(ptr_) {}

	virtual ~sgobject() {
		std::visit([](auto&& arg){arg.reset();}, ptr);
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