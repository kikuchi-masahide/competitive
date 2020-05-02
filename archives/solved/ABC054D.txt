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

int N, MA, MB;
struct medic {
	int a, b;
	int c;
};
vector<medic> medics;
int NA, NB;
vector<vector<int>> deck;

int main() {
	cin >> N >> MA >> MB;
	NA = N / 2;
	NB = N - NA;
	medics.resize(N);
	for (int n = 0; n < N; n++) {
		cin >> medics[n].a >> medics[n].b >> medics[n].c;
	}
	deck.resize(201, vector<int>(201, INT_MAX));
	for (int code = 0; code < (1 << NB); code++) {
		int tempcost = 0, tempa = 0, tempb = 0;
		for (int n = 0; n < NB; n++) {
			if (((code >> n) & 1) == 1) {
				tempcost += medics[NA + n].c;
				tempa += medics[NA + n].a;
				tempb += medics[NA + n].b;
			}
		}
		deck[tempa][tempb] = min(deck[tempa][tempb], tempcost);
	}
	int ans = INT_MAX;
	for (int code = 0; code < (1 << NA); code++) {
		int tempcost = 0, tempa = 0, tempb = 0;
		for (int n = 0; n < NA; n++) {
			if (((code >> n) & 1) == 1) {
				tempcost += medics[n].c;
				tempa += medics[n].a;
				tempb += medics[n].b;
			}
		}
		int k = max(ceil((long double)tempa / MA), ceil((long double)tempb / MB));
		k = max(k, 1);
		while (true) {
			if (k * MA - tempa > 200 || k * MB - tempb > 200)break;
			if (deck[k * MA - tempa][k * MB - tempb] != INT_MAX)ans = min(ans, tempcost + deck[k * MA - tempa][k * MB - tempb]);
			k++;
		}
	}
	if (ans == INT_MAX)cout << -1 << endl;
	else cout << ans << endl;
	return 0;
}