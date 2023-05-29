import subprocess

def clean_wheels():
    command = "rm -rf target/wheels/*"
    subprocess.run(command, shell=True)

def build_wheels():
    clean_wheels()
    command = "pip uninstall lana && maturin build --sdist && unset CONDA_PREFIX && maturin develop"
    subprocess.run(command, shell=True)

def publish():
    command = "python -m twine upload --repository pypi target/wheels/*"
    subprocess.run(command, shell=True)

if __name__ == "__main__":

    build_wheels()