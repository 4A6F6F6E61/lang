#include <iostream>
auto gtest = 10;
const auto ctest = 20;
void print (int msg)
{
std::cout << msg << std::endl ;
}
#include <vector>

std::vector<int> gener ()
{
std::vector<int> dfjfjfdjfndjfnjd;
dfjfjfdjfndjfnjd.push_back(10);
dfjfjfdjfndjfnjd.push_back(10);
return dfjfjfdjfndjfnjd;
}
int main ()
{
for(auto test:gener())
{
print (test );
}
}