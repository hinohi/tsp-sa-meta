import sys
import subprocess

import optuna


BIN = './target/release/opt_2d'

TOWNS = int(sys.argv[1])
SIZE = int(sys.argv[2])
DIST = sys.argv[3]
COUNT = int(sys.argv[4])
N = int(sys.argv[5])

def evaluate(temp_max, temp_min, exponent, swap_rate):
    s = 0.0
    for i in range(N):
        args = [
            BIN,
            '--seed', str(i * 2 + 2),
            '--towns', str(TOWNS),
            '--size', str(SIZE),
            '--dist', str(DIST),
            '--count', str(COUNT),
            '--temp-max', str(temp_max),
            '--temp-min', str(temp_min),
            '--exponent', str(exponent),
            '--swap-rate', str(swap_rate),
        ]
        p = subprocess.Popen(args, stdout=subprocess.PIPE)
        p.wait()
        out, err = p.communicate()
        s += float(out)
    return s / N


def objective(trial):
    temp_max = trial.suggest_uniform('temp_max', 10.0, 10000.0)
    temp_min = trial.suggest_uniform('temp_min', 2 ** -10, 10.0)
    exponent = trial.suggest_uniform('exponent', 2 ** -10, 10.0)
    swap_rate = trial.suggest_uniform('swap_rate', 0.0, 1.0)
    return evaluate(temp_max, temp_min, exponent, swap_rate)


def main():
    name = f'{TOWNS}-{SIZE}-{DIST}-{COUNT}'
    study = optuna.create_study(
        study_name=name,
        storage=f'sqlite:///{name}.db',
        load_if_exists=True,
    )
    study.optimize(objective, n_trials=1000, n_jobs=4)


if __name__ == "__main__":
    main()
