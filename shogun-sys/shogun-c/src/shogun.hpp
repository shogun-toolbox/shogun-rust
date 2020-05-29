#include <stdint.h>

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

	struct float64_result
	{
		RETURN_CODE return_code;
		union ResultFloat64Union
		{
			double result;
			const char* error;
		} result;
	};

	struct Result
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
		SG_FEATURES,
		SG_FILE,
		SG_COMBINATION_RULE,
		SG_LABELS,
		SG_EVALUATION,
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
	Result sgobject_put(sgobject_t*, const char*, const void*, TYPE);
	Result sgobject_put_array(sgobject_t*, const char*, const void*, uint32_t, uint32_t, TYPE);
	SG_TYPE sgobject_derived_type(const sgobject_t*);

	sgobject_result create_machine(const char*);
	Result train_machine(sgobject_t*, sgobject_t*);
	sgobject_result apply_machine(sgobject_t*, sgobject_t*);
	sgobject_result apply_multiclass_machine(sgobject_t*, sgobject_t*);


	sgobject_result create_kernel(const char*);
	Result init_kernel(sgobject_t*, sgobject_t*, sgobject_t*);

	sgobject_result create_distance(const char*);

	sgobject_result create_features(const char*);
	sgobject_result create_features_from_data(const void*, uint32_t rows, uint32_t cols, TYPE);
	sgobject_result create_features_from_file(const sgobject_t*);

	sgobject_result create_labels(const char*);
	sgobject_result create_labels_from_file(const sgobject_t*);

	sgobject_result create_file(const char*);
	sgobject_result read_csvfile(const char*);

	sgobject_result create_combination_rule(const char*);

	sgobject_result create_evaluation(const char*);
	float64_result evaluate_labels(sgobject_t*, sgobject_t*, sgobject_t*);

	void set_parallel_threads(int32_t);
}