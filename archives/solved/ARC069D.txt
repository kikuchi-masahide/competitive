void solve();

int main() {
	solve();
	return 0;
}

//////////////////////////////////////////////////
//////////////////////////////////////////////////
#include <iostream>
#include <vector>
#include <algorithm>
#include <string>
using namespace std;
typedef pair<int, int> pii;

void solve() {
	int N;
	cin >> N;
	vector<bool> input;
	input.resize(N);
	string S; cin >> S;
	for (int n = 0; n < N; n++) {
		if (S[n] == 'o')input[n] = true;
		else input[n] = false;
	}
	vector<bool> res;   //true -> S,false -> W
	res.resize(N);
	bool flag = false;
	for (int i1 = 0; i1 <= 1 && !flag; i1++) {
		for (int i2 = 0; i2 <= 1 && !flag; i2++) {
			res[0] = i1;
			res[1] = i2;
			for (int n = 2; n < N; n++) {
				res[n] = ~(res[n-1] ^ ~(res[n-2] ^ input[n-1]));
			}
			if (res[0] == ~(res[N - 1] ^ ~(res[N - 2] ^ input[N - 1]))) {
				if (res[1] == ~(res[N - 1] ^ ~(res[0] ^ input[0]))) {
					flag = true;
					for (int n = 0; n < N; n++) {
						cout << (res[n]?"S":"W");
					}
					cout << endl;
				}
			}
		}
	}
	if (!flag)cout << -1 << endl;
	return;
}