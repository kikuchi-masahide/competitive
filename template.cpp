//#pragma warning(disable:4996)
//#include <Windows.h>
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
#include <cassert>
#include <boost/multiprecision/cpp_int.hpp>
#include <complex>
#include <cstdio>
#include <list>
#include <bitset>
//#include <stdio.h>

//< in.txt > out.txt
using namespace std;
//std::ios::sync_with_stdio(false);
//std::cin.tie(0);
const long long MOD = 1e9 + 7;
const long long INF = 1e18;
typedef long long LL;
typedef long double LD;
typedef boost::multiprecision::cpp_int bigint;
typedef pair<LL, LL> PLL;
typedef pair<int, int> PI;
typedef pair<LD, LL> pdl;
typedef pair<LD, LD> pdd;
typedef vector<LL> VLL;
typedef vector<VLL> VVLL;
typedef vector<int> VI;
typedef vector<vector<int>> VVI;
typedef unsigned long long ULL;
template<class T>
using pqueue = priority_queue<T, vector<T>, function<bool(T, T)>>;

template<class T>
inline void chmin(T& a, T b) {
	a = min(a, b);
}

template<class T>
inline void chmax(T& a, T b) {
	a = max(a, b);
}

void input();
void solve();

void daminput();
void naive();

void outputinput();

int main() {
	std::cin.tie(0);
	std::ios::sync_with_stdio(false);
	input();
	//daminput();
	solve();
	//naive();
	//outputinput();
	return 0;
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////

void input() {
}

void daminput() {
}

class A {
public:
	int id;
	A(int _id)
		:id(_id)
	{
		cout << "A.const:id=";
		cout << id << "\n";
	}
	~A() {
		cout << "A.dest:id=" << id << "\n";
	}
};

class AA :public A {
public:
	AA(int _id)
		:A(_id)
	{
		cout << "AA.const\n";
	}
	~AA() {
		cout << "AA.dest\n";
	}
};

template<class T>
class H {
public:
	H(T* _t)
		:t(_t){
		cout << "const\n";
	}
	H(const H<T>& h) {
		cout << "copy\n";
		t = h.t;
	}
	template<class U>
	explicit operator H<U>() const noexcept {
		cout << "cast:left's id:" << t->id << "\n";
		return H<U>((U*)t);
	}
	H<T>& operator=(const H<T>& h) {
		cout << "=\n";
		t = h.t;
	}
private:
	T* t;
};

void solve() {
	H<A> h1(new A(0));
	H<AA> h2(new AA(1));
	h1 = h2;
}

void naive() {
}

void outputinput() {
}