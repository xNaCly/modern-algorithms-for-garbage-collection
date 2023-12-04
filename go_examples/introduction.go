// see Section Introduction
package main

import "fmt"

type Person struct {
	Name string
	Age  float64
}

func NewPerson(name string, age float64) *Person {
	return &Person{name, age}
}

func main() {
	for i := 0; i < 1e5; i++ {
		p := NewPerson("max musterman", 89)
		fmt.Printf("Person{name: %q, age: %f}\n", p.Name, p.Age)
	}
}
