mkdir build
cd build
cmake -G "Visual Studio 9 2008" -A Win32 -S .. -B "build32"
cmake --build build32 --config Release --target _t_name_t_ 
cd ../