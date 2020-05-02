#include <iostream>
#include <vector>
#include <limits.h>
#include <algorithm>
#include <string>
#include <math.h>
#include <limits.h>
#include <queue>
#include <map>
#include <set>
#include <iomanip>
#include <bitset>
#include <cassert>
#include <random>
#include <functional>
#include <stack>
#include <iomanip>
using namespace std;

template<class T>
void in(vector<T>& V) {
	for (auto itr = V.begin(); itr != V.end(); itr++)cin >> *itr;
	return;
}

template<class T>
void out(vector<T>& V) {
	for (T& itr : V)cout << itr << endl;
	return;
}

int N;
vector<int> mark;   //-1:未 0:vacant 1:male 2:female

int main() {
	string input;
	cin >> N;
	mark.resize(N,-1);
	cout << 0 << endl;
	cout << flush;
	cin >> input;
	if (input == string("Vacant"))return 0;
	else if (input == string("Male"))mark[0] = 1;
	else mark[0] = 2;
	int s = 0, e = N - 1;
	while (e - s > 1) {
		int m = (s + e) / 2;
		cout << m << endl;
		cout << flush;
		cin >> input;
		if (input == string("Vacant"))return 0;
		else if (input == string("Male"))mark[m] = 1;
		else mark[m] = 2;
		if (s % 2 == m % 2) {
			if (mark[s] == mark[m]) s = m;
			else e = m;
		}
		else {
			if (mark[s] == 3 - mark[m])s = m;
			else e = m;
		}
	}
	cout << s << endl;
	cout << flush;
	cin >> input;
	if (input == string("Vacant"))return 0;
	cout << e << endl;
	cout << flush;
	cin >> input;
	if (input == string("Vacant"))return 0;
	return 0;
}