// see Garbage Collection/Strategies/Escape Analysis
package main

type T struct{ x int64 }

func A() *T {
	return &T{x: 12}
}

func B() {
	t := &T{x: 25}
	t.x++
}

func main() {
	A()
	B()
}
