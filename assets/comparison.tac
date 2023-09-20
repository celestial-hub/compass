# a < b && c > d
      if a < b goto L1
      t1 = 0
      goto L2
L1:   t1 = 1
L2:   if c > d goto L3
      t2 = 0
      goto L4
L3:   t2 = 1
L4:   t3 = t1 && t2

end: # 'result' is in t3
