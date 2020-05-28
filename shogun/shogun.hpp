extern "C" {
	struct version;
	struct sgobject;
	struct C_Visitor;

	typedef struct version version_t;
	typedef struct sgobject sgobject_t;
	typedef struct C_Visitor cvisitor_t;
	
	enum RETURN_CODE {SUCCESS, ERROR};

	struct sgobject_result
	{
		RETURN_CODE return_code;
		union ResultUnion
		{
			sgobject_t* result;
			const char* error;
		} result;
	};

	struct sgobject_put_result
	{
		RETURN_CODE return_code;
		const char* error;
	};
	

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

	TYPE get_cvisitor_type(const cvisitor_t*);
	const char* get_cvisitor_typename(const cvisitor_t*);
	void* get_cvisitor_pointer(const cvisitor_t*);

	version_t* create_version();
	void destroy_version(version_t*);
	const char* get_version_main(version_t*);

	void destroy_sgobject(sgobject_t*);
	const char* to_string(const sgobject_t*);
	cvisitor_t* sgobject_get(const sgobject_t*, const char*);
	sgobject_put_result sgobject_put(sgobject_t*, const char*, const void*, TYPE);
	SG_TYPE sgobject_derived_type(const sgobject_t*);

	sgobject_result create_machine(const char*);
	bool train_machine(sgobject_t*);

	sgobject_result create_kernel(const char*);

	sgobject_result create_distance(const char*);
}