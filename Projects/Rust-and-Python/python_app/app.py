import ctypes
import time
import os


# Load the Rust dynamic library based on the operating system
def load_rust_library():
    lib_dir = os.path.join(os.path.dirname(__file__), "../rust_lib/target/release")
    if os.name == "nt":  # Windows
        lib_path = os.path.join(lib_dir, "math_lib.dll")
    elif os.name == "posix":  # Linux or macOS
        if os.uname().sysname == "Darwin":  # macOS
            lib_path = os.path.join(lib_dir, "libmath_lib.dylib")
        else:  # Linux
            lib_path = os.path.join(lib_dir, "libmath_lib.so")

    return ctypes.CDLL(lib_path)


# Load the library and set up function arguments and return types
math_lib = load_rust_library()
math_lib.sum_of_squares.argtypes = [ctypes.POINTER(ctypes.c_int), ctypes.c_int]
# Adjust the return type to a C-style string pointer
math_lib.sum_of_squares.restype = ctypes.POINTER(ctypes.c_char)

# Add a function to free the memory
math_lib.free_string.argtypes = [ctypes.POINTER(ctypes.c_char)]
math_lib.free_string.restype = None


# Python implementation of the same function
def python_sum_of_squares(arr):
    total = 0
    for num in arr:
        total += num * num
    return total


if __name__ == "__main__":
    data_size = 1000000
    numbers = list(range(data_size))

    start_time = time.perf_counter()
    python_result = python_sum_of_squares(numbers)
    end_time = time.perf_counter()
    python_time = end_time - start_time
    print(f"Python Result: {python_result}")
    print(f"Python Time: {python_time:.4f} seconds\n")

    # Prepare data for Rust call
    c_numbers = (ctypes.c_int * data_size)(*numbers)

    # Use Rust function
    start_time = time.perf_counter()
    result_ptr = math_lib.sum_of_squares(c_numbers, data_size)

    # Convert the C-style string pointer to a Python string
    rust_result = ctypes.string_at(result_ptr).decode("utf-8")
    end_time = time.perf_counter()
    rust_time = end_time - start_time

    # Print results
    print(f"Rust Result: {rust_result}")
    print(f"Rust Time: {rust_time:.4f} seconds")

    # Free the memory allocated by Rust
    math_lib.free_string(result_ptr)

    # Performance comparison
    if rust_time > 0:
        speed_up = python_time / rust_time
        print(
            f"\nPerformance Improvement: Rust is {speed_up:.2f} times faster than Python."
        )
