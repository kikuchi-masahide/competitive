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
//#include <boost/multiprecision/cpp_int.hpp>
#include <complex>
#include <cstdio>
#include <list>
#include <bitset>
//#include <stdio.h>

//< in.txt > out.txt
using namespace std;
//std::ios::sync_with_stdio(false);
//std::cin.tie(0);
const long long mod1e9 = 1000000000 + 7;
const long long mod998244353 = 998244353;
const long long INF = 1e18;
typedef long long LL;
typedef long double LD;
typedef unsigned long long ULL;
//typedef boost::multiprecision::cpp_int bigint;
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
string alphabet = "abcdefghijklmnopqrstuvwxyz";

template<class T>
inline void chmin(T& a, T b) {
	a = min(a, b);
}

template<class T>
inline void chmax(T& a, T b) {
	a = max(a, b);
}

//y/xÇÃfloorÇãÅÇﬂÇÈ
LL floor_(LL y, LL x) {
	if (x < 0) {
		x *= -1;
		y *= -1;
	}
	if (y >= 0) {
		return y / x;
	}
	else {
		if ((-y) % x == 0) {
			return y / x;
		}
		else {
			return -((-y) / x) - 1;
		}
	}
}

inline LL mod(LL a, LL m) {
	LL res = a % m;
	return res >= 0 ? res : res + m;
}

void input();
void solve();

void daminput();
void naive();

void outputinput();

int main() {
	std::cin.tie(0);
	std::ios::sync_with_stdio(false);
	cout << fixed << setprecision(12);
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

void solve() {
}

void naive() {
}

void outputinput() {
}