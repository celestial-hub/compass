# Initialize variables
      t0 = 0              # t0 stores 'a', initialize to 0
      t1 = 1              # t1 stores 'b', initialize to 1
      t2 = 0              # t2 stores 'i', initialize to 0
      t3 = 0              # t3 stores 'result', initialize to 0
      t4 = n              # t4 stores 'n', get value from argument

# Start of loop
loop: if t2 >= t4 goto end  # If 'i' >= 'n', exit loop
      t3 = t0               # 'result' = 'a'
      t5 = t0 + t1          # Compute next Fibonacci number
      t0 = t1               # Shift 'a'
      t1 = t5               # Shift 'b'
      t6 = t2 + 1           # Increment loop counter
      t2 = t6               # Update 'i'
      goto loop
# End of loop

end:  # 'result' is in t3
