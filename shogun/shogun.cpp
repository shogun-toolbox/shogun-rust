#include "shogun.h"
#include <shogun/base/Version.h>
#include <shogun/util/factory.h>

using namespace shogun;

struct version {
	std::unique_ptr<Version> obj;
};

struct machine {
	std::shared_ptr<Machine> machine;
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

machine_t* create_machine(const char* name) {
	auto* ptr = (machine_t*)malloc(sizeof(machine_t));
	auto obj = create<Machine>(name);
	ptr->machine = std::move(obj);
	return ptr;
}

void destroy_machine(machine_t* ptr) {
	if (ptr) {
		ptr->machine.reset();
		free(ptr);
	}
}

const char* to_string(const machine_t* ptr) {
	if (ptr) {
		auto repr = ptr->machine->to_string();
		auto* result = (char*)malloc(sizeof(char*) * repr.size() + 1);
		strcpy(result, repr.c_str());
		return result;
	}
}