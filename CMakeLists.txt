cmake_minimum_required(VERSION 3.20)
project(frameflow-rust NONE)  # no C/C++ compiler needed for the Rust side

set(MYLIB_LIB ${CMAKE_CURRENT_SOURCE_DIR}/target/release/libframeflow_rs.a)

set(MYLIB_HEADER ${CMAKE_CURRENT_SOURCE_DIR}/include/frameflow-rs.h)

find_program(CBINDGEN_EXECUTABLE cbindgen REQUIRED)

add_custom_command(
    OUTPUT ${MYLIB_HEADER}
    COMMAND ${CMAKE_COMMAND} -E make_directory ${CMAKE_CURRENT_SOURCE_DIR}/include
    COMMAND ${CBINDGEN_EXECUTABLE} --config cbindgen.toml --crate frameflow_rs --output ${MYLIB_HEADER}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    DEPENDS
        ${CMAKE_CURRENT_SOURCE_DIR}/src/lib.rs
        ${CMAKE_CURRENT_SOURCE_DIR}/cbindgen.toml
    COMMENT "Generating frameflow-rs.h with cbindgen"
)
add_custom_target(mylib_header DEPENDS ${MYLIB_HEADER})

add_custom_command(
    OUTPUT ${MYLIB_LIB}
    COMMAND cargo build --release
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    DEPENDS ${CMAKE_CURRENT_SOURCE_DIR}/src/lib.rs ${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml
    COMMENT "Building Rust library"
)
add_custom_target(mylib_build DEPENDS ${MYLIB_LIB})

add_library(frameflow_rs STATIC IMPORTED GLOBAL)
set_target_properties(frameflow_rs PROPERTIES IMPORTED_LOCATION ${MYLIB_LIB})
target_include_directories(frameflow_rs INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/include)
add_dependencies(frameflow_rs mylib_build mylib_header)

# System libraries Rust's std needs. Replace these with whatever
# `cargo rustc --release -- --print native-static-libs` reports on your machine.
if(APPLE)
    # target_link_libraries(frameflow_rs INTERFACE "-framework Security" "-framework CoreFoundation" System)
else()
    # target_link_libraries(frameflow_rs INTERFACE pthread dl m)
endif()
