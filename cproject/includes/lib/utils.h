#pragma once
#include <cstdint>
#include <type_traits>

namespace rc {
  namespace utils {
    struct T;
  }
}

namespace rc {
namespace utils {
#ifndef CXXBRIDGE1_STRUCT_rc$utils$T
#define CXXBRIDGE1_STRUCT_rc$utils$T
struct T final {
  ::std::int32_t a;
  ::std::int32_t b;

  ::std::int32_t add() const noexcept;
  using IsRelocatable = ::std::true_type;
};
#endif // CXXBRIDGE1_STRUCT_rc$utils$T

::rc::utils::T n() noexcept;
} // namespace utils
} // namespace rc
