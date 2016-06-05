#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
extern crate libc;
use std::ffi::CString;

pub type jint = i32;
pub type jdouble = f64;

pub enum _jobject {}
pub type jobject = *mut _jobject;

pub enum _jclass {}
pub type jclass = *mut _jclass;

pub enum _jmethodID {}
pub type jmethodID = *mut _jmethodID;

pub struct jvalue(jobject);

pub type UnmappedFunction = *const libc::c_void;

#[repr(C)]
pub struct JNINativeInterface {
    reserved0: UnmappedFunction,
    reserved1: UnmappedFunction,
    reserved2: UnmappedFunction,
    reserved3: UnmappedFunction,
    GetVersion: extern "C" fn(*mut JNIEnv) -> jint,
    DefineClass: UnmappedFunction,
    FindClass: UnmappedFunction,
    FromReflectedMethod: UnmappedFunction,
    FromReflectedField: UnmappedFunction,
    ToReflectedMethod: UnmappedFunction,
    GetSuperclass: UnmappedFunction,
    IsAssignableFrom: UnmappedFunction,
    ToReflectedField: UnmappedFunction,
    Throw: UnmappedFunction,
    ThrowNew: UnmappedFunction,
    ExceptionOccurred: UnmappedFunction,
    ExceptionDescribe: UnmappedFunction,
    ExceptionClear: UnmappedFunction,
    FatalError: UnmappedFunction,
    PushLocalFrame: UnmappedFunction,
    PopLocalFrame: UnmappedFunction,
    NewGlobalRef: UnmappedFunction,
    DeleteGlobalRef: UnmappedFunction,
    DeleteLocalRef: UnmappedFunction,
    IsSameObject: UnmappedFunction,
    NewLocalRef: UnmappedFunction,
    EnsureLocalCapacity: UnmappedFunction,
    AllocObject: UnmappedFunction,
    NewObject: UnmappedFunction,
    NewObjectV: UnmappedFunction,
    NewObjectA: UnmappedFunction,
    GetObjectClass: extern "C" fn (*mut JNIEnv, obj: jobject) -> jclass,
    IsInstanceOf: UnmappedFunction,
    GetMethodId: extern "C" fn (*mut JNIEnv, class: jclass, name: *const libc::c_char, sig: *const libc::c_char) -> jmethodID,
    CallObjectMethod: UnmappedFunction,
    CallObjectMethodV: UnmappedFunction,
    CallObjectMethodA: extern "C" fn (*mut JNIEnv, obj: jobject, methodID: jmethodID, args: *const jvalue) -> jobject
}

#[repr(C)]
pub struct JNIEnv {
    functions: *mut JNINativeInterface
}

impl JNIEnv {
    fn _this(&mut self) -> *mut Self {
        self as *mut Self
    }

    fn GetMethodID(&mut self, class: jclass, name: &str, sig: &str) -> jmethodID {
        let name = CString::new(name).unwrap();
        let sig = CString::new(sig).unwrap();


        unsafe { ((*self.functions).GetMethodId)(self._this(), class, name.as_ptr(), sig.as_ptr()) }
    }

    fn GetObjectClass(&mut self, obj: jobject) -> jclass {
        unsafe { ((*self.functions).GetObjectClass)(self._this(), obj) }
    }

    fn GetVersion(&mut self) -> jint {
        unsafe { ((*self.functions).GetVersion)(self._this()) }
    }

    fn CallObjectMethod(&mut self, obj: jobject, methodID: jmethodID, args: &[jvalue]) -> jobject {
        unsafe { 
            let pargs = args.as_ptr();
            ((*self.functions).CallObjectMethodA)(self._this(), obj, methodID, pargs)
        }
    }
}

#[no_mangle]
pub extern fn Java_HelloJNI_sayHello(_: *mut JNIEnv, _: jobject) {
    println!("Hello from Rust!");
}

#[no_mangle]
pub extern fn Java_HelloJNI_average(_: *mut JNIEnv, _: jobject, a: jint, b: jint) -> jdouble {
    let da = a as jdouble;
    let db = b as jdouble;
    (da+db)/2.0
}

#[no_mangle]
pub unsafe extern fn Java_HelloJNI_callFromRust(env: *mut JNIEnv, this: jobject) {
    println!("We will call a Java method from within Rust...");
    println!("API Version: {}", (*env).GetVersion());

    let thisClass = (*env).GetObjectClass(this);
    let methodID = (*env).GetMethodID(thisClass, "showFromJava", "()V");

    (*env).CallObjectMethod(this, methodID, &[]);
}
