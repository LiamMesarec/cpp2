# CPP2 (WIP)

## Features

<table>
<tr>
<td>Feature</td><td> Cpp2 </td> <td> Result </td> <td> Implementation Status </td>
</tr>
<tr>
<td> Project Creation </td>
<td>
cpp2 new projectname --cmake
</td>
<td>
Creates a new CMake project with a main.cpp
</td>
<td> 

- [ ] WIP

</td>  
</tr>
<tr>
<td> Project Building </td>
<td>
cpp2 build
</td>
<td>
Compiles the project into cpp files 
</td>
<td>
  
- [ ] WIP
</td>  
</tr>
<tr>
<td> Project Running </td>
<td>
cpp2 run
</td>
<td>
Runs (and builds if necessary) the project with the default setup 
</td>
<td>
  
- [ ] WIP
</td>  
</tr>
<tr>
<td> Auto-Fetch Libraries </td>
<td>
cpp2 add --git=git_link --tag=tag --name=library_name
</td>
<td>
Edits the CMakeLists with FetchContent
  
```cmake
FetchContent_Declare(
    library_name
    GIT_REPOSITORY git_link
    GIT_TAG tag
    CONFIGURE_COMMAND ""
    BUILD_COMMAND ""
)

FetchContent_MakeAvailable(library_name)
```

</td>
<td> 
  
- [ ] WIP
</td>  
</tr>
<tr>
<td> Const By Default </td>
<td> 

```cpp
let i: u32 = u
```

```cpp
let mut j: u32 = 10
j = j + 10
```

</td>
<td>
    
```cpp
#include <cstdint>
...
const uint_32t i = u;
```

```cpp
uint_32t j = 10;
j = j + 10;
```

</td>
<td> 
  
- [ ] WIP
</td>  
</tr>
<tr>
<td> First Class Functions </td>
<td>
  
```cpp
let even = fn (i: i32): i32 {
  return 0 == i % 2
}
```

</td>
<td>
    
```cpp
auto even = [](int32_t i) {
  return 0 == i % 2;
};
```

</td>
<td>
  
- [ ] WIP
</td>  
</tr>
<tr>
<td> Functions </td>
<td>
  
```cpp
fn func(t: i32): i32 {
  return t
}
```

</td>
<td>
    
```cpp
int32_t func(int32_t t) {
  return t;
}
```

</td>
<td>

- [ ] WIP
</td>  
</tr>
<tr>
<td> Header and Source Generation </td>
<td>
  
```cpp
fn func(t: i32): i32 {
  return t;
}
```

</td>
<td>
.h
  
```cpp
int32_t func(int32_t t);
```
.cpp
  
```cpp
int32_t func(int32_t t) {
  return t;
}
```

</td>
<td>

- [ ] WIP
</td>  
</tr>
<tr>
<td> File-Scoped Namespaces </td>
<td>
  
```cpp
namespace A::B

fn func(t: i32): i32 {
  return t
}
```

</td>
<td>
    
```cpp
namespace A::B {
  int32_t func(int32_t t) {
    return t;
  }
}
```

</td>
<td>

- [ ] WIP
</td>  
</tr>
<tr>
<td> Templates </td>
<td>
  
```cpp
fn func<T>(t: T): i32 {
  return t;
}

```

</td>
<td>
  
```cpp
template<typename T>
T func(T t) {
  return t;
}
```

</td>
<td>

- [ ] WIP
</td>  
</tr>
<tr>
<td> Async </td>
<td>

</td>
<td>

</td>
<td>

- [ ] WIP
</td>  
</tr>
<tr>
<td> Range-Based For Loops </td>
<td>
  
```cpp
for i: i32 in 0..10 {

}
```

```cpp
for &x in array {

}
```

</td>
<td>

```cpp
for(int32_t i = 0; i < 10; i++) {

}
```

```cpp
for(const auto& x : array) {

}
```

</td>
<td>

- [ ] WIP
</td>  
</tr>
<tr>
<td> If Statements </td>
<td>
  
```cpp
if i < 10 {

} else if i > 10 {

}
```

</td>
<td>

```cpp
if (i < 10) {

} else if (i > 10) {

}
```

</td>
<td>

- [ ] WIP
</td>  
</tr>
<tr>
<td> Async </td>
<td>

</td>
<td>

</td>
<td>

- [ ] WIP
</td>  
</tr>
</table>
