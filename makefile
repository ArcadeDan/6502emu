final:
	g++ -std=c++2a -c -g -Wall *.cpp
	g++ -std=c++2a -g -Wall *.o
	rm *.o