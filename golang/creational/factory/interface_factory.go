package main

import "fmt"

type Person interface {
	SayHello()
}

type person struct {
	name string
	age  int
}

func (p *person) SayHello() {
	fmt.Printf("Hi, my name is %s, I am %d years old\n",
		p.name, p.age)
}

func NewPerson(name string, age int) Person {
	return &person{name, age}
}

func main() {
	// initialize with factory method
	p := NewPerson("James", 34)
	// Can't do somethiong like p.name or p.age
	p.SayHello()
}
