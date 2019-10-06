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

long long N, K;

int main() {
	cin >> N >> K;
	if (K == 0) {
		cout << N*N << endl;
		return 0;
	}
	long long ans = 0;
	for (int b = K + 1; b <= N; b++) {
		ans += (b - K) * (N / b) + max((long long)0,(N%b)-K+1);
	}
	cout << ans << endl;
	return 0;
}
