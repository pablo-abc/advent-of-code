(ns core
  (:require ["node:fs" :as fs]
            [clojure.string :refer [split]]))

(defn count-larger [m]
  (->>
   (mapv #(if (> %2 %1) 1 0) m (subvec m 1))
   (apply +)))

(defn get-m-windows [m]
  (loop [wnds []
         mes m]
    (if (< (count mes) 3)
      wnds
      (recur
       (conj
        wnds
        (apply + (subvec (into [] mes) 0 3)))
       (rest mes)))))

(->
 (fs/readFileSync "./input.txt"  #js {:encoding "utf-8"})
 (split #"\n")
 (->>
  (mapv js/Number))
 (get-m-windows)
 (count-larger)
 (print))
