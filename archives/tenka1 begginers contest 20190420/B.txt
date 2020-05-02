void solve();

int main() {
	solve();
	return 0;
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
#include <iostream>
#include <string>
using namespace std;

void solve() {
	int N, K;
	string S;
	cin >> N >> S >> K;
	for (int n = 0; n < N; n++) {
		if (S[n] != S[K - 1])cout << "*";
		else cout << S.substr(n,1);
	}
	cout << endl;
	return;
}