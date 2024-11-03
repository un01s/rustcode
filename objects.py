# python has the garbage collector. 
# python passes objects around "by reference"
# 
class Person:
  def __init__(self, name):
    self.name = name
    self.friends = []

  def add_friend(self, other):
    self.friends.append(other)

alice = Person("Alice")
bob = Person("Bob")
alice.add_friend(bob)
bob.add_friend(alice)

