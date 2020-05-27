extern "C" {
	struct version;
	struct sgobject;
	
	typedef struct version version_t;
	typedef struct sgobject sgobject_t;

	version_t* create_version();
	void destroy_version(version_t*);
	const char* get_version_main(version_t*);

	void destroy_sgobject(sgobject_t*);

	sgobject_t* create_machine(const char*);
	bool train_machine(sgobject_t*);

	sgobject_t* create_kernel(const char*);

	const char* to_string(const sgobject_t*);
}