(def input1
  (map (fn [s]
         (let [digits (clojure.string/split s #"")]
           (map #(Integer/parseInt %) digits)))
    (line-seq (clojure.java.io/reader "input"))))

(def sums
  (reduce (partial map +) input))


(defn s1 []
  (let [sums (reduce (partial map +) input)
        half (/ (count input) 2)]
    (->> sums
         (reduce (fn [[g e] sum]
                   [(str g (if (> sum half) 1 0))
                    (str e (if (< sum half) 1 0))]) ["" ""])
         (map #(Integer/parseInt % 2))
         (apply *))))

(s1)


(def input2
  (line-seq (clojure.java.io/reader "input")))

(defn s2 [input i compare-fn]
  (if (= 1 (count input))
    (Integer/parseInt (first input) 2)

    (let [{g0 \0 g1 \1} (group-by #(get % i) input)]
      (recur
        (if (compare-fn (count g0) (count g1)) g0 g1)
        (inc i)
        compare-fn))))

(* (s2 input2 0 >) (s2 input2 0 <=))
