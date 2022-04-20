/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode(int x) : val(x), next(NULL) {}
 * };
 */
class Solution {
public:
    void deleteNode(ListNode* node) {
        //since we can't get the parent node, we must change the value to it's child's, and delete its child node
        node->val = node->next->val;
        ListNode* to_delete = node->next;
        node->next = node->next->next;
        delete to_delete;
    }
};
