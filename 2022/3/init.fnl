(var rucksack-sum 0)
(local groups [{}])
(var badge-sum 0)

(fn get-priority [char]
  (let [is-lower-case (string.match char "[a-z]")
        byte-char (string.byte char)]
    (if is-lower-case
       (- byte-char 96)
       (- byte-char 38))))

(var line-index 0)
(each [line (io.lines)]
  (set line-index (+ 1 line-index))

  ;; Part 1
  (let [found-comp-chars {}
        split-pos (/ (length line) 2)
        first-compartment (string.sub line 1 split-pos)
        second-compartment (string.sub line (+ 1 split-pos))]
    (for [i 1 split-pos]
      (let [char (string.sub first-compartment i i)
            is-in-both (string.find second-compartment char)]
        (if (and is-in-both (not (. found-comp-chars char)))
            (set rucksack-sum (+ rucksack-sum (get-priority char))))
        (tset found-comp-chars char true))))

  ;; Part 2
  (let [found-badge-chars {}]
    (table.insert (. groups (length groups)) line)
    (if (= (% line-index 3) 0)
        (let [group (. groups (length groups))]
          (for [i 1 (length (. group 1))]
            (let [char (string.sub (. group 1) i i)
                  is-in-all (and
                              (string.find (. group 2) char)
                              (string.find (. group 3) char))]
              (if (and is-in-all (not (. found-badge-chars char)))
                  (set badge-sum (+ badge-sum (get-priority char))))
              (tset found-badge-chars char true)))
          (table.insert groups {})))))

(print "Part 1:" rucksack-sum)
(print "Part 2:" badge-sum)
