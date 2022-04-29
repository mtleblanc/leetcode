func canBeTypedWords(text string, brokenLetters string) int {
	var bad = make(map[rune]struct{})
	var present = struct{}{}
	for _, l := range brokenLetters {
		bad[l] = present
	}

	var words = strings.Split(text, " ")
	var total int = len(words)

	for _, w := range words {
		for _, l := range w {
			_, prs := bad[l]
			if prs {
				total -= 1
				break
			}
		}
	}
	return total
} 
