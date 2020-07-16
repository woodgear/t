#include <stdio.h>
#include "demo.h"

int main(int argc, char const *argv[])
{
    printf("%s\n",Demo().echo("hello world!").c_str());
    return 0;
}
