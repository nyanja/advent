(def input
  (map (fn [s]
         (let [[d n] (clojure.string/split s #" ")]
           [d (Integer/parseInt n)]))
    (line-seq (clojure.java.io/reader "input"))))


(defn s1 []
  (apply * (reduce
             (fn [[depth position] [direction length]]
               (case direction
                 "up"      [(- depth length) position]
                 "down"    [(+ depth length) position]
                 "forward" [depth (+ position length)]))
             [0 0] input)))

(s1)


(defn s2 []
  (apply * (rest
             (reduce
               (fn [[aim depth position] [direction value]]
                 (case direction
                   "up"      [(- aim value) depth position]
                   "down"    [(+ aim value) depth position]
                   "forward" [aim (+ (* aim value) depth) (+ position value)]))
               [0 0 0] input))))

(s2)
