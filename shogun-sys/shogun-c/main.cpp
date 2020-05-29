#include "shogun.hpp"
#include <iostream>

int main() {
    auto* version = create_version();
    std::cout << get_version_main(version) << '\n';
    destroy_version(version);
}