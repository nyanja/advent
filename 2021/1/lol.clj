(def input
  (map #(Integer/parseInt %) (line-seq (clojure.java.io/reader "input"))))


(defn a1 [input]
  (loop [[h & t] input
         i       0]
    (if (seq t)
      (recur t (if (< h (first t))
                 (inc i)
                 i))
      i)))

(a1 input)


(defn a2 []
  (loop [[a & t] input
         res     []]
    (if (<= 2 (count t))
      (let [[b & [c]] t]
        (recur t (conj res (+ a b c))))
      (a1 res))))

(a2)
