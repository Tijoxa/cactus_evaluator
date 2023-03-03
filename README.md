# cactus_evaluator
Implémentation de l'algorithme Cactus Kev Perfect Hash.

# Dépendances
- [Rust](https://www.rust-lang.org/fr)
- maturin (pip install maturin)

# Installation
## Par la source
Nécessite [Rust](https://www.rust-lang.org/fr)
<pre>pip install maturin
maturin develop</pre>

## Par wheels
<pre>pip install .\cactus_evaluator-0.1.1-cp310-none-win_amd64.whl</pre>

# Utilisation
- Utilisation dans Python :
```Python
import cactus_evaluator
cactus_evaluator.evaluate('AC', '4C', '5C', 'TC', 'AH')
>>> 3484
cactus_evaluator.evaluate_7('AC', '4C', '5C', 'TC', 'AH', '9H', '7S')
>>> 3463
```

# Source
- [Cactus Kev Algorithm](https://suffe.cool/poker/evaluator.html)
- [Hash Tables](http://suffe.cool/poker/7462.html)
- [PyO3](https://github.com/PyO3/pyo3)