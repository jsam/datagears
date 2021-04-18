import os
import shutil
from pathlib import Path

import pytest

from datagears.core.package import Package, chdir


class TestPackage:
    """Tests for packager."""

    def test_setup_name(self) -> None:
        """Check constant name."""
        assert Package.SETUP_NAME == "setup.py"

    def test_children(self) -> None:
        """Check children results."""
        children = Package.children(Path("."))
        assert Package.SETUP_NAME in children

        with pytest.raises(ValueError):
            Package.children(Path("setup.py"))

    def test_package_constructor(self) -> None:
        """Check package construction."""
        _cwd = Path(os.getcwd())
        package = Package()
        assert package.root == _cwd

        os.chdir("datagears/core")
        package_child = Package()
        assert package_child.root == _cwd

        os.chdir(f"{_cwd}/..")
        with pytest.raises(ValueError):
            Package()

        os.chdir(_cwd)

    def test_package_chdir(self) -> None:
        """Test chdir context manager."""
        _cwd = Path(os.getcwd())

        with chdir("datagears/core"):
            assert Path(os.getcwd()) == _cwd / "datagears" / "core"

        assert Path(os.getcwd()) == _cwd

    def test_make_egg(self) -> None:
        """Test make egg."""
        package = Package()
        egg_path = package.make_egg()
        assert egg_path.exists()

        dst = Path("".join([*str(egg_path).split(".egg"), "_copy", ".egg"]))
        shutil.copyfile(egg_path, dst)

        _rmtree = shutil.rmtree
        shutil.rmtree = lambda x: x  # type: ignore
        with pytest.raises(ValueError) as exp:
            package.make_egg()
        shutil.rmtree = _rmtree

        assert str(exp.value) == "multiple eggs found"

        _cwd = package._cwd  # type: ignore
        package._cwd = None  # type: ignore
        with pytest.raises(ValueError) as exp:
            package.make_egg()

        assert str(exp.value) == "cannot find setup.py"

        Package.BUILD_EGG_CMD = "python setup.py sdist"  # type: ignore
        package._cwd = _cwd  # type: ignore

        with pytest.raises(ValueError) as exp:
            package.make_egg()

        assert str(exp.value) == "egg was not built"
