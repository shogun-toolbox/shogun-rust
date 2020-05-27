extern "C" {
	struct version;
	struct sgobject;
	struct C_Visitor;

	enum TYPE {
		INT32,
		INT64,
		FLOAT32,
		FLOAT64,
		SGOBJECT,
	};
	
	enum SG_TYPE {
		SG_KERNEL,
		SG_MACHINE,
		SG_DISTANCE,
	};

	typedef struct version version_t;
	typedef struct sgobject sgobject_t;
	typedef struct C_Visitor cvisitor_t;

	TYPE get_cvisitor_type(const cvisitor_t*);
	const char* get_cvisitor_typename(const cvisitor_t*);
	void* get_cvisitor_pointer(const cvisitor_t*);

	version_t* create_version();
	void destroy_version(version_t*);
	const char* get_version_main(version_t*);

	void destroy_sgobject(sgobject_t*);
	const char* to_string(const sgobject_t*);
	cvisitor_t* sgobject_get(const sgobject_t*, const char*);
	SG_TYPE sgobject_derived_type(const sgobject_t*);

	sgobject_t* create_machine(const char*);
	bool train_machine(sgobject_t*);

	sgobject_t* create_kernel(const char*);

	sgobject_t* create_distance(const char*);
}