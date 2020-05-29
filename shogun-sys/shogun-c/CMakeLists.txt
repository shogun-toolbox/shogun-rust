find_package(spdlog 1.3.1 REQUIRED)
find_package(Shogun REQUIRED)

add_library(shogun-c STATIC src/shogun.cpp)
set_target_properties(shogun-c PROPERTIES 
                      CXX_STANDARD 17)
                      
target_link_libraries(shogun-c PRIVATE Shogun::shogun-static spdlog::spdlog)

target_include_directories(shogun-c PRIVATE)

install(TARGETS shogun-c)