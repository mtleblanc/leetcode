/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func mergeInBetween(list1 *ListNode, a int, b int, list2 *ListNode) *ListNode {
    cursor := list1
    for a > 1 {
        a --;
        b --;
        cursor = cursor.Next
    }
    insertAfter := cursor
    for b >= 0 {
        b --;
        cursor = cursor.Next
    }
    tail := list2
    for tail.Next != nil {
        tail = tail.Next
    }
    insertAfter.Next = list2
    tail.Next = cursor
    return list1
}
