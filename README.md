# Interfacing Rust with the JVM

Since Rust provides a great interface with C, it is possible to use Java's C bindings (the Java Native Interface, or JNI) to interact with the JVM from within Rust.

Suppose we have a Java class like the following, which invokes a foreign function called ```sayHello``` stored in an external library called ```hello```:

```java
public class HelloJNI {
    static {
        System.loadLibrary("hello");
    }

    private native void sayHello();

    public static void main(String[] args) {
        HelloJNI jni = new HelloJNI();

        jni.sayHello();
    }
}
```

Java will expect a dynamic library called ```hello``` (either ```libhello.so``` or ```hello.dll``` depending on whether you are on UNIX or Windows) on the ```java.library.path``` with a special function defined.

Following Java's conventions, the function must be named ```Java_<class _name>_<method_name>```, in this case ```Java_HelloJNI_sayHello```. The function is passed at least two parameters: a pointer of type ```JNIEnv``` which is a virtual function table that interacts with the JVM, and another pointer of type ```jobject``` which refers to the ```this``` object in Java. Refer to [the JNI documentation](http://docs.oracle.com/javase/8/docs/technotes/guides/jni/spec/jniTOC.html) for more information.

So, how can we use this information to interact with Rust?

First, we need to create a standard dynamic library with our Rust functions so they can be called from "outside". This is easily done by defining the ```crate-type``` property as ```dylib``` in our ```Cargo.toml``` file, like this:

```toml
[package]
name = "hello"
(...)

[lib]
crate-type = ["dylib"]
```

Now, every function we want to export to the JVM must have its name de-mangled with the ```#[no_mangle]``` directive, and it has to follow the proper naming convention:

```rust
// In src/lib.rs
#[no_mangle]
pub extern fn Java_HelloJNI_sayHello(_: *mut JNIEnv, _: jobject) {
    println!("Hello from Rust!");
}
```

Now, what are ```JNIENV``` and ```jobject``` in this code? Easy: These are just mutable pointers to opaque structures. In rust, we define opaque structures as empty enums:

```rust
pub enum JNIEnv {}

// This is just to make it look more like the JNI interface
pub enum _jobject {}
pub type jobject = *mut _jobject;
```

These opaque pointers are all we need to interact with the JVM. The pointer to the ```JNIEnv``` contains a list of functions to call the JVM API. See the ```jni.h``` file to have an idea of what this API contains.

Unfortunately, there is an extra issue we need to overcome if we want to do something useful. For some reason, the JVM API is not invoked using functions directly, but rather by calling functions stored in the ```JNIEnv``` function table. We need then to replicate JNI's function table within Rust.

Again, translating most of the code found in the ```jni.h``` header file, we can do the following:

```rust
// This will mark the functions that we have not mapped yet
pub type UnmappedFunction = *const libc::c_void;

#[repr(C)]
pub struct JNINativeInterface {
    reserved0: UnmappedFunction,
    reserved1: UnmappedFunction,
    reserved2: UnmappedFunction,
    reserved3: UnmappedFunction,
    GetVersion: extern "C" fn(*mut JNIEnv) -> jint,
    // ... etc
    GetMethodId: extern "C" fn (*mut JNIEnv, class: jclass, name: *const libc::c_char, sig: *const libc::c_char) -> jmethodID,
    /// ... etc
  }

#[repr(C)]
pub struct JNIEnv {
    functions: *mut JNINativeInterface
}
```

Again, this is just a translation of what we find in ```jni.h```.

So, how do we call the JNI API? Well, we are given a pointer to a ```JNIEnv``` object, so to get a pointer to the API functions we will need to de-reference this pointer and then de-reference the pointer to the ```JNINativeInterface``` to get the actual pointer to the function. For example, suppose we want to invoke the ```GetVersion``` JNI API function to retrieve the API version number. We have to do the following:

```rust
let version = ((*jniEnv.functions).GetVersion)(jniEnv)
```

The code in this repository contains a couple of (non-exhaustive) examples of how to call Rust functions from the JVN and viceversa.

## How to run the examples
Make sure you have the JDK and Cargo installed, and just run:

```bash
./run.sh
```

You should see an output like the following:

```
Hello from Rust!
The average is: 7.5
We will call a Java method from within Rust...
API Version: 65544
This method is in Java
Finished
```

The code in this repository is just a proof-of-concept, it does not intend to be a JNI wrapper for Rust, although this can be done with little effort. I think that combining this with the power of compiler plugins could allow us to make interfacing with the JVM a breeze!
