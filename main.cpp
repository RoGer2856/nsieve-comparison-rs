// The Computer Language Shootout
// http://shootout.alioth.debian.org/
// Precedent C entry modified by bearophile for speed and size, 31 Jan 2006
// Converted to C++ by Paul Kitchin

#include <iomanip>
#include <iostream>
#include <sstream>
#include <vector>
#include <concepts>
#include <ranges>
#include <algorithm>
#include <limits>

using namespace std;
using namespace std::views;

// void nsieve(std::size_t max) {
//   static std::vector<bool> flags;
//   flags.assign(max, false);
//   std::size_t count = 0;
//   for (std::size_t value = 2; value < max; ++value) {
//     if (!flags[value]) {
//       ++count;
//       for (std::size_t multiple = value * 2; multiple < max;
//            multiple += value) {
//         flags[multiple] = true;
//       }
//     }
//   }
//   std::cout << "Primes up to " << std::setw(8) << max << ' ' << std::setw(8)
//             << count << '\n';
// }

template <unsigned_integral T>
void nsieve( T n)
{
	vector<bool> flags(n);
	T count = 0u;
	for ( T i : iota( static_cast<T>(2), n) )
	{
		if (!flags[i])
		{
			++count;
			for ( T j = i * 2; j < n; j += i)
			{
				flags[j] = true;
			}
		}
	}
	cout<<"Primes up to "<<setw(8)<<n<<' '<<setw(8)<<count<<'\n';
}

void dispatch_sieve(size_t n)
{
	if (n < numeric_limits<uint32_t>::max() / 2)
	{
		nsieve<uint32_t>( n);
	}
	else
	{
		nsieve<size_t>( n);
	}
}

int main(int argc, char **argv) {
  if (argc != 2) {
    std::cerr << "usage: " << argv[0] << " <n>\n";
    return 1;
  }
  unsigned int count;
  {
    std::istringstream convertor(argv[1]);
    if (!(convertor >> count) || !convertor.eof()) {
      std::cerr << "usage: " << argv[0] << " <n>\n";
      std::cerr << "\tn must be an integer\n";
      return 1;
    }
  }
  for (std::size_t i = 0; i < 3; ++i) {
    dispatch_sieve(10000 << (count - i));
  }
}
