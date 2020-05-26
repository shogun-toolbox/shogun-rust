extern "C" {
	struct version;
	struct machine;

	typedef struct version version_t;
	typedef struct machine machine_t;

	version_t* create_version();
	void destroy_version(version_t*);
	const char* get_version_main(version_t*);

	machine_t* create_machine(const char*);
	void destroy_machine(machine_t*);
	bool train_machine(machine_t*);

	const char* to_string(const machine_t*);
}