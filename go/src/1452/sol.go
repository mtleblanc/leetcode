package main

import "fmt"

type bitmask struct {
	lo uint64
	hi uint64
}

func main() {
	fmt.Println("hi")
}
func peopleIndexes(favoriteCompanies [][]string) []int {
	seen := make(map[string]int)
	numCompanies := 0
	numPeople := len(favoriteCompanies)
	var allCompanies []bitmask

	for person := 0; person < numPeople; person++ {
		companies := favoriteCompanies[person]
		for favoriteIdx := 0; favoriteIdx < len(companies); favoriteIdx++ {
			company := companies[favoriteIdx]
			idx, ok := seen[company]
			if !ok {
				idx = numCompanies
				seen[company] = numCompanies
				allCompanies = append(allCompanies, bitmask{0, 0})
				numCompanies++
			}
			if person >= 64 {
				allCompanies[idx].hi |= (1 << (person - 64))
			} else {
				allCompanies[idx].lo |= (1 << person)
			}
		}
	}

	notSubset := make([]bitmask, numPeople)
	def := bitmask{0, 0}
	if numPeople <= 64 {
		def.hi = (1 << 64) - 1
		if numPeople < 64 {
			def.lo = ^((1 << numPeople) - 1)
		}
	} else {
		def.hi = ^((1 << (numPeople - 64)) - 1)
	}
	for person := 0; person < numPeople; person++ {
		notSubset[person] = def
		if person < 64 {
			notSubset[person].lo |= (1 << person)
		} else {
			notSubset[person].hi |= (1 << (person - 64))
		}
	}

	for companyIdx := 0; companyIdx < numCompanies; companyIdx++ {
		mask := allCompanies[companyIdx]
		notMask := mask
		notMask.lo = ^notMask.lo
		notMask.hi = ^notMask.hi
		for k := 0; k < numPeople; k++ {
			if (k < 64 && (mask.lo&(1<<k)) != 0) || (k >= 64 && mask.hi&(1<<(k-64)) != 0) {
				notSubset[k].lo |= notMask.lo
				notSubset[k].hi |= notMask.hi
			}
		}
	}

	var ret []int
	for person := 0; person < numPeople; person++ {
		mask := notSubset[person]
		if (^mask.hi == 0) && (^mask.lo == 0) {
			ret = append(ret, person)
		}
	}
	return ret
}
