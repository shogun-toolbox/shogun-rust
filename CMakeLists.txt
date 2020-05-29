project(shogun-c)

add_library(shogun-c STATIC shogun/shogun.cpp)
set_target_properties(shogun-c PROPERTIES 
                      CXX_STANDARD 17)
                      
target_link_libraries(shogun-c /home/gf712/shogun/build/src/shogun/libshogun.a /home/gf712/shogun/build/spdlog/src/SpdLog-build/libspdlog.a)

target_include_directories(shogun-c PRIVATE
                           /home/gf712/shogun/build/src
                           /home/gf712/shogun/src
                           /home/gf712/shogun/third_party/spdlog/include/
                           )

install(TARGETS shogun-c)