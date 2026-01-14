cmake_minimum_required(VERSION 3.25)
get_filename_component(CMAKE_PROJECT_NAME ${CMAKE_SOURCE_DIR} NAME_WE)
project(${CMAKE_PROJECT_NAME} VERSION 0.0.1 LANGUAGES C CXX ASM)

include(src)      # scan project for source code files

add_library(${CMAKE_PROJECT_NAME} STATIC ${C} ${CP})

install(TARGETS ${CMAKE_PROJECT_NAME} DESTINATION .)
