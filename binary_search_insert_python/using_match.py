# 35. Search Insert Position
#
# Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
# 
# You must write an algorithm with O(log n) runtime complexity.


def searchInsert(nums: list[int], target: int) -> int:

    l = 0
    r = len(nums)-1
    i = r // 2

    while l <= i <= r:
        match nums[i]:
            case _ if nums[i] == target: return i           # Target found; return index of target
            case _ if nums[i] > target: r = i - 1           # Too small; set l to be 1 more than current index i
            case _ if nums[i] < target: l = i + 1           # Too big; set r to be 1 less than current index i
                
        i = (l+r) // 2                                      # Centre index i to be roughly midpoint of l & r

    # Return l, because here, target was not found, and l will always be equal to max(l,r) as
    # while condition has been broken
    return l


n = [1,2,5,10]
t = 5
print(f'Index is { searchInsert(n, t) }')
