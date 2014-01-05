tim: tim.rs datatypes.rs
	rustc tim.rs

timopt: tim.rs
	rustc -O -o timopt tim.rs