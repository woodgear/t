cmake_minimum_required (VERSION 3.17+)

set (PROJECT_NAME "_t_name_t_")

project (${PROJECT_NAME})

add_compile_definitions("UNICODE")
add_compile_definitions("_UNICODE")

add_executable(${PROJECT_NAME} 
    ./src/main.cpp
    ./src/demo.cpp
)

target_include_directories(${PROJECT_NAME}
PRIVATE 
    ./src
)
