#include <iostream>
auto gtest = 10;
const auto ctest = 20;
void print (int msg)
{
std::cout << msg << std::endl ;
}
#include <vector>

std::vector<int> test ()
{
std::vector<int> dfjfjfdjfndjfnjd;
dfjfjfdjfndjfnjd.push_back(10);
dfjfjfdjfndjfnjd.push_back(666);
return dfjfjfdjfndjfnjd;
}
int main ()
{
auto array = test();
print (array [0 ]);
print (array [1 ]);
return 0;
}