
namespace rc {
namespace utils {

extern "C" {

/// Function adding a int with another int and return an int
int add(int a, int b);

void print_str(const char *text, const char *values);

void print_int(const char *text, const int *values);

void print_bool(const char *text, const bool *values);

}  // extern "C"

}  // namespace utils
}  // namespace rc
