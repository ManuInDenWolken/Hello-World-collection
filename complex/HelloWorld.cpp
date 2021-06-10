#include <iostream>
#include <algorithm>
#include <memory>

namespace sayHello
{
    template<class T>
    class Say
    {
        protected:
            std::string msg;
            T data;
        public:
            Say(std::string text = "Hi!", T d = "No data available") : msg(text), data(d) {}
            virtual void operator() () noexcept = 0;
            virtual ~Say() = default;
    };
    class HelloWorld : public Say<std::string>
    {
        public:
            explicit HelloWorld() : Say("Hello, ", "World!\n") {};
            void operator() () noexcept final override;
    };
    void HelloWorld::operator() () noexcept
    {
        for(auto c: this->msg) {
            std::cout << c;
        }
        std::for_each(this->data.begin(), this->data.end(), [] (char c)
        {
            std::cout << c;
        });
    }
}

int main(void)
{
    std::unique_ptr<sayHello::Say<std::string>> hello(new sayHello::HelloWorld());
    (*hello)();
}
