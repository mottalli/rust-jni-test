all:	target/debug/libhello.so HelloJNI.class

target/debug/libhello.so:	src/lib.rs
	cargo build

HelloJNI.class:	HelloJNI.java
	javac HelloJNI.java
